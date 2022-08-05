{ pkgs, ... }:
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
      # TODO: fix this race condition
      service.command = "sh -c '${pkgs.coreutils}/bin/sleep 5 && while !</dev/tcp/postgres/5432; do ${pkgs.coreutils}/bin/sleep 5; done; ${flake.packages.x86_64-linux.default}/bin/discord-chess'";
      service.env_file = [ "bot.env" ];
      service.restart = "on-failure";
      image.enableRecommendedContents = true;

      service.depends_on = [
        "postgres"
      ];
    };
  };
}
