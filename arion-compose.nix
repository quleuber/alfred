{ pkgs, ... }:
{
  project.name = "alfred";
  services = {

    traefik = {
      service = {
        image = "traefik:v2.10";
        command = [ "--api.insecure=true" "--providers.docker" ];
        ports = [
          "80:80"
          "443:443"
          "8080:8080"
        ];
        volumes = [
          "/var/run/docker.sock:/var/run/docker.sock"
          "./data/traefik/:/etc/traefik/"
        ];
      };
    };

    webserver = {
      image.enableRecommendedContents = true;
      service.useHostStore = true;
      service.command = [ "sh" "-c" ''cd "$$WEB_ROOT"; ${pkgs.python3}/bin/python -m http.server'' ];
      service.ports = [
        "8000:8000" # host:container
      ];
      service.environment.WEB_ROOT = "${pkgs.nix.doc}/share/doc/nix/manual";
      service.stop_signal = "SIGINT";
    };
  };
}
