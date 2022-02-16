{ rustPlatform, dbus_libs, pkgconfig, lib }:
rustPlatform.buildRustPackage {
  pname = "idasen-cli";
  version = "0.1.0";

  src = ./.;

  buildInputs = [ dbus_libs ];
  nativeBuildInputs = [ pkgconfig ];

  cargoHash = "sha256-B/kdwykveYTB2K/sLKO0Bznc5cIxP9E3zKnYGzZxIyM=";

  meta = with lib; {
    description = ''Control your IKEA® IDÅSEN standing desk by a cli tool using Bluetooth.
(This is neither made nor endorsed by IKEA®.)'';
    homepage = "https://github.com/typetetris/idasen-cli";
    license = licenses.mit;
    maintainers = [ maintainers.typetetris ];
  };

}
