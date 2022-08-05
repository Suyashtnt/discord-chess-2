{  ... }:
let 
  flake = builtins.getFlake (toString ./.);
in
{
  services = {
    postgres = {
      service.image = "postgres:latest";
      service.env_file = [ "db.env" ];
      service.volumes = [ "${toString ./.}/postgresql-data:/var/lib/postgresql/data" ];
      service.ports = [ "5432:5432" ];
    };

    bot = {
      service.useHostStore = true;
      service.ports = [ "8080:8080" "6669:6669" ];
      service.command = [ "${flake.packages.x86_64-linux.default}/bin/discord-chess" ];
      service.env_file = [ "bot.env" ];
    };
  };
}
