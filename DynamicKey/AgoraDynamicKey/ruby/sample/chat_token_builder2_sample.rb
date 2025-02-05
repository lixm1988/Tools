require_relative '../lib/dynamic_key2'

app_id = '970CA35de60c44645bbae8a215061b33'
app_certificate = '5CFd2fd1755d40ecb72977518be15d3b'
account = "test"
token_expiration_in_seconds = 3600

token = AgoraDynamicKey2::ChatTokenBuilder.build_user_token(app_id, app_certificate, account, token_expiration_in_seconds)
puts "Chat user token : #{token}"

token = AgoraDynamicKey2::ChatTokenBuilder.build_app_token(app_id, app_certificate, token_expiration_in_seconds)
puts "Chat App token: #{token}"
