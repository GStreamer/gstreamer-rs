$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

# Download gstreamer and all its subprojects
git clone -b $env:DEFAULT_BRANCH --depth 1 https://gitlab.freedesktop.org/gstreamer/gstreamer.git C:\gstreamer
if (!$?) {
  Write-Host "Failed to clone gstreamer"
  Exit 1
}

Set-Location C:\gstreamer

# Make use of the subprojects cache
python C:/gstreamer/ci/scripts/handle-subprojects-cache.py --cache-dir /subprojects C:/gstreamer/subprojects

# Update the subprojects cache
Write-Output "Running meson subproject reset"
meson subprojects update --reset
if (!$?) {
  Write-Host "Failed to update gstreamer subprojects"
  Exit 1
}

$MESON_ARGS = @(`
  "--prefix=C:\gst-install", `
  "-Dglib:installed_tests=false", `
  "-Dlibnice:tests=disabled", `
  "-Dlibnice:examples=disabled", `
  "-Dffmpeg:tests=disabled", `
  "-Dopenh264:tests=disabled", `
  "-Dpygobject:tests=false", `
  "-Dgpl=enabled", `
  "-Dugly=enabled", `
  "-Dbad=enabled", `
  "-Dges=enabled", `
  "-Drtsp_server=enabled", `
  "-Ddevtools=enabled", `
  "-Dsharp=disabled", `
  "-Dpython=disabled", `
  "-Dlibav=disabled", `
  "-Dgtk=enabled", `
  "-Dgst-plugins-base:pango=enabled", `
  "-Dgst-plugins-good:cairo=enabled", `
  "-Dgst-plugins-good:lame=disabled"
)

Write-Output "Building gstreamer"
meson setup --vsenv $MESON_ARGS _build
if (!$?) {
  type "_build\meson-logs\meson-log.txt"
  Write-Host "Failed to run meson setup, see log above"
  Exit 1
}

Write-Output "Compiling gstreamer"
meson compile -C _build
if (!$?) {
  Write-Host "Failed to run meson compile"
  Exit 1
}
# meson install does a spurious rebuild sometimes that then fails
meson install --no-rebuild -C _build
if (!$?) {
  Write-Host "Failed to run meson install"
  Exit 1
}

cd c:\
Remove-Item -LiteralPath "C:\gstreamer" -Force -Recurse
