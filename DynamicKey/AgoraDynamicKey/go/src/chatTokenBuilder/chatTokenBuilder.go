package chatTokenBuilder

import (
	accesstoken2 "github.com/AgoraIO/Tools/DynamicKey/AgoraDynamicKey/go/src/accesstoken2"
)

// BuildChatUserToken method
// @param appID: The App ID issued to you by Agora. Apply for a new App ID from
// Agora Dashboard if it is missing from your kit. See Get an App ID.
// @param appCertificate: Certificate of the application that you registered in
// the Agora Dashboard. See Get an App Certificate.
// @param userUuid: The user's id, must be unique.
// @param expire: represented by the number of seconds elapsed since now. If, for example, you want to access the Agora Service within 10 minutes after the token is generated, set expire as 600(seconds).
//
// @return The Chat User token.
func BuildChatUserToken(appID string, appCertificate string, userUuid string, expire uint32) (string, error) {
	accessToken2 := accesstoken2.NewAccessToken(appID, appCertificate, expire)

	serviceChat := accesstoken2.NewServiceChat(userUuid)
	serviceChat.AddPrivilege(accesstoken2.PrivilegeChatUser, expire)
	accessToken2.AddService(serviceChat)

	token, err := accessToken2.Build()
	return token, err
}

// BuildChatAppToken method
// @param appID: The App ID issued to you by Agora. Apply for a new App ID from
// Agora Dashboard if it is missing from your kit. See Get an App ID.
// @param appCertificate: Certificate of the application that you registered in
// the Agora Dashboard. See Get an App Certificate.
// @param expire: represented by the number of seconds elapsed since now. If, for example, you want to access the Agora Service within 10 minutes after the token is generated, set expire as 600(seconds).
//
// @return The Chat App token.
func BuildChatAppToken(appID string, appCertificate string, expire uint32) (string, error) {
	accessToken2 := accesstoken2.NewAccessToken(appID, appCertificate, expire)

	serviceChat := accesstoken2.NewServiceChat("")
	serviceChat.AddPrivilege(accesstoken2.PrivilegeChatApp, expire)
	accessToken2.AddService(serviceChat)

	return accessToken2.Build()
}
