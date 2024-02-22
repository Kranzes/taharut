{ config, lib, pkgs, ... }:

let
  cfg = config.services.taharut;
in
{

  options.services.taharut = {
    enable = lib.mkEnableOption "taharut.org excersize notifier";

    package = lib.mkPackageOption pkgs "taharut" { };

    settings.interval = lib.mkOption {
      type = lib.types.ints.u16;
      default = 180;
      description = "How often (in minutes) to check for new exercises";
    };
  };


  config = lib.mkIf cfg.enable {
    systemd.user.services.taharut = {
      Unit = {
        Description = "taharut - taharut.org excersize notifier";
        After = [ "graphical-session-pre.target" ];
        PartOf = [ "graphical-session.target" ];
      };
      Service = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/taharut --interval ${toString cfg.settings.interval}";
        Restart = "on-failure";
      };
      Install.WantedBy = [ "graphical-session.target" ];
    };
  };
}
