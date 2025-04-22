#!/usr/bin/env perl
use strict;
use warnings;
use utf8;
use 5.010;

use FindBin;

use lib $FindBin::Bin."/../src";

use Agora::SignalingToken;

# Need to set environment variable AGORA_APP_ID
my $app_id                = $ENV{AGORA_APP_ID};
# Need to set environment variable AGORA_APP_CERTIFICATE
my $app_certificate       = $ENV{AGORA_APP_CERTIFICATE};

my $account               = "2882341273";
my $now                   = time();
my $valid_time_in_seconds = 3600*24;
my $expired_ts_in_seconds = $now + $valid_time_in_seconds;

if (!defined($app_id) || $app_id eq "" || !defined($app_certificate) || $app_certificate eq "") {
    say "Need to set environment variable AGORA_APP_ID and AGORA_APP_CERTIFICATE";
    exit 1;
}
say "App Id: $app_id";
say "App Certificate: $app_certificate";

my $token = Agora::SignalingToken::gen_signaling_token($account, $app_id, $app_certificate, $expired_ts_in_seconds);
say "Token: $token";
