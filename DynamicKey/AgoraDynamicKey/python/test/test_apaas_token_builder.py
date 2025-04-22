# -*- coding: utf-8 -*-
__copyright__ = "Copyright (c) 2014-2024 Agora.io, Inc."

import sys
import unittest
import os
from src.apaas_token_builder import *

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))


class ApaasTokenBuilderTest(unittest.TestCase):
    def setUp(self):
        self.__app_id = "970CA35de60c44645bbae8a215061b33"
        self.__app_cert = "5CFd2fd1755d40ecb72977518be15d3b"
        self.__room_uuid = "123"
        self.__user_id = "2882341273"
        self.__role = 1
        self.__expire = 900

    def test_room_user_token(self):
        token = ApaasTokenBuilder.build_room_user_token(
            self.__app_id, self.__app_cert, self.__room_uuid, self.__user_id, self.__role, self.__expire)
        parser = AccessToken()
        parser.from_string(token)

        self.assertEqual(parser._AccessToken__app_id, self.__app_id.encode('utf-8'))
        self.assertEqual(parser._AccessToken__expire, self.__expire)
        self.assertIn(ServiceApaas.kServiceType, parser._AccessToken__service)

        parser_service = parser._AccessToken__service[ServiceApaas.kServiceType]

        self.assertEqual(parser_service._ServiceApaas__room_uuid, self.__room_uuid.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__user_uuid, self.__user_id.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__role, self.__role)
        self.assertIn(ServiceApaas.kPrivilegeRoomUser, parser_service._Service__privileges)

    def test_user_token(self):
        token = ApaasTokenBuilder.build_user_token(
            self.__app_id, self.__app_cert, self.__user_id, self.__expire)
        parser = AccessToken()
        parser.from_string(token)

        self.assertEqual(parser._AccessToken__app_id, self.__app_id.encode('utf-8'))
        self.assertEqual(parser._AccessToken__expire, self.__expire)
        self.assertIn(ServiceApaas.kServiceType, parser._AccessToken__service)

        parser_service = parser._AccessToken__service[ServiceApaas.kServiceType]
        self.assertEqual(parser_service._ServiceApaas__room_uuid, ''.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__user_uuid, self.__user_id.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__role, -1)
        self.assertIn(ServiceApaas.kPrivilegeUser, parser_service._Service__privileges)

    def test_app_token(self):
        token = ApaasTokenBuilder.build_app_token(
            self.__app_id, self.__app_cert, self.__expire)
        parser = AccessToken()
        parser.from_string(token)

        self.assertEqual(parser._AccessToken__app_id, self.__app_id.encode('utf-8'))
        self.assertEqual(parser._AccessToken__expire, self.__expire)
        self.assertIn(ServiceApaas.kServiceType, parser._AccessToken__service)

        parser_service = parser._AccessToken__service[ServiceApaas.kServiceType]
        self.assertEqual(parser_service._ServiceApaas__room_uuid, ''.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__user_uuid, ''.encode('utf-8'))
        self.assertEqual(parser_service._ServiceApaas__role, -1)
        self.assertIn(ServiceApaas.kPrivilegeApp, parser_service._Service__privileges)
