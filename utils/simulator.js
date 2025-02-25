// utils/simulator.js

const config = require("../utils/config");
const puppeteer = require("puppeteer");

let basicAuthValue = null;
let bearerAuthValue = null;

const startSimulator = async (rawUcode) => {
  const requestUcode = rawUcode || config.testStudentRawUcode;

  const browser = await puppeteer.launch({
    headless: true,
    defaultViewport: null,
    args: [
      "--no-sandbox",
      "--disable-setuid-sandbox",
      "--disable-gpu",
      "--disable-dev-shm-usage",
    ],
  });

  const addAuthValue = (type, value) => {
    switch (type) {
      case "basic":
        basicAuthValue = value;
        break;
      case "bearer":
        bearerAuthValue = value;
        break;
      default:
        break;
    }
  }

  const page = await browser.newPage();
  await page.setRequestInterception(true);

  page.on("request", async (request) => {
    const headers = request.headers();
    const authorization = headers["authorization"];

    if (authorization) {
      // Basic 认证
      if (authorization.startsWith("Basic")) {
        addAuthValue('basic', authorization);
      }

      // Token 认证
      else if (authorization.startsWith("Bearer")) {
        addAuthValue('bearer', authorization);
      }
    }

    request.continue();
  });

  try {
    await page.goto(
      `${config.collegeAppBaseUrl}/czmobile/mytimetableIndexNew?uid=${requestUcode}`,
      {
        waitUntil: "networkidle0",
      }
    );

    console.log(`Success: 模拟器已加载 ${requestUcode} 的数据。`);
  } catch (error) {
    if (error.message.includes('ERR_CONNECTION_CLOSED')) {
      console.log("Error: 页面加载失败，连接已关闭。");
    } else {
      console.log("出现错误:", error.message);
    }
  } finally {
    await browser.close();
  }

  return { basicAuthValue, bearerAuthValue };
}

module.exports = { startSimulator };
