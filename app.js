"use strict";

const fs = require("fs");
const path = require("node:path");
const AutoLoad = require("@fastify/autoload");

const options = {};

module.exports = async function (fastify, opts) {
  // 自动加载插件
  fastify.register(AutoLoad, {
    dir: path.join(__dirname, "plugins"),
    options: Object.assign({}, opts),
  });

  // 自动加载路由
  fastify.register(AutoLoad, {
    dir: path.join(__dirname, "routes"),
    options: {
      prefix: "/api",
    },
  });

  // 注册静态文件服务
  fastify.register(require("@fastify/static"), {
    root: path.join(__dirname, "public"),
    prefix: "/",
  });

  // 捕获所有未匹配的请求
  fastify.setNotFoundHandler(async (request, reply) => {
    const filePath = path.join(__dirname, "public", request.raw.url);
    try {
      await fs.promises.access(filePath, fs.constants.F_OK);
      return reply.sendFile(request.raw.url.substring(1));
    } catch (err) {
      if (err.code === "ENOENT") {
        return reply.sendFile("index.html");
      } else {
        reply.code(500).send(err);
      }
    }
  });
};

module.exports.options = options;
