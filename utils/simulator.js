// utils/simulator.js

const config = require("../utils/config");
const puppeteer = require("puppeteer");

let basicAuthValue = null;
let bearerAuthValue = null;

const startSimulator = async () => {
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

  const page = await browser.newPage();
  await page.setRequestInterception(true);

  page.on("request", (request) => {
    const headers = request.headers();
    const authorization = headers["authorization"];

    if (authorization) {
      // Basic 认证
      if (authorization.startsWith("Basic")) {
        basicAuthValue = authorization;
        if (basicAuthValue && bearerAuthValue) browser.close();
      }

      // Token 认证
      else if (authorization.startsWith("Bearer")) {
        bearerAuthValue = authorization;
        if (basicAuthValue && bearerAuthValue) browser.close();
      }
    }

    request.continue();
  });

  await page.goto(
    `${config.collegeAppBaseUrl}/czmobile/mytimetableIndexNew?uid=${config.testStudentRawUcode}`, { waitUntil: "networkidle0" }
  );

  console.log(`✅ 模拟器已加载 ${config.testStudentRawUcode} 的数据。`);
  
  return { basicAuthValue, bearerAuthValue };
}

module.exports = { startSimulator };
