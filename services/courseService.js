// services/courseService.js

const axios = require("axios");

const config = require("../utils/config");
const authService = require("./authService");

/**
 * 传入 UCode 以获得用户 Token
 * @param {*} rawUcode
 */
const getUserToken = async (rawUcode) => {
  const requestUrl = config.collegeAppBaseUrl + "/gateway/auth/oauth/token";
  const ucode = `HUA_TENG-${rawUcode}`;

  const requestBody = {
    ucode,
    state: 1,
    grant_type: ucode,
    scope: server,
  };
};
