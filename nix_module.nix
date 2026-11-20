{
  config,
  lib,
  inputs,
  ...
}:

with lib;
with lib.types;
let
  cfg = config.services.whether;
in
{
  options = {
    services.whether = {
      enable = mkEnableOption "whether";
      package = lib.mkOption {
        type = types.package;
        default = inputs.whether.packages.x86_64-linux.default;
      };
    };
  };

  config = mkIf cfg.enable {
    systemd.timers.whether = {
      description = "reMarkable weather pdf generator";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnBootSec = "5m";
        OnUnitActiveSec = "5m";
        Unit = "whether.service";
      };
    };

    systemd.services."whether" = {
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${lib.getExe cfg.package}";
        User = "eva";
        Group = "users";
        # Prevents the service from automatically starting on rebuild
        RemainAfterExit = true;
      };
      environment = {
        XDG_CONFIG_HOME = "/home/eva/.config";
      };
    };

  };
}
