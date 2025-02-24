// services/authService.js

const config = require("../utils/config");
const simulator = require("../utils/simulator");

/**
 * 获取 Basic 验证字符串（根据校内服务器数据推测，存在不确定性）
 * @returns {string}
 */
const getBasicAuth = () => {
  const username = "cat";
  const password = "cat";
  return "Basic " + Buffer.from(`${username}:${password}`).toString("base64");
};

/**
 * 获取 Basic 验证字符串（模拟浏览器环境，直接模拟学生访问课表以获取现实数据，非必要不用）
 * @param {string} ucode 
 * @returns {string}
 */
const getServerBasicAuth = async (ucode) => {
  const { basicAuthValue } = await startSimulator();
  return basicAuthValue;
};
