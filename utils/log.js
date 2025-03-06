'use strict';

const fs = require('fs');
const path = require('path');

const LOG_DIR = path.join(__dirname, '../logs');

if (!fs.existsSync(LOG_DIR)) {
    fs.mkdirSync(LOG_DIR, { recursive: true });
}

/**
 * 生成日志文件的路径
 * 
 * 该函数通过结合日志目录（LOG_DIR）和当前日期的ISO字符串格式（仅日期部分）来生成日志文件的完整路径
 * 这样做可以确保每天的日志都被写入到单独的文件中，便于日志管理和查询
 * 
 * @returns {string} 日志文件的路径，格式为"<LOG_DIR>/<日期>.log"
 */
const getLogFilePath = () => path.join(LOG_DIR, `${new Date().toISOString().split('T')[0]}.log`);

/**
 * 获取当前时间戳
 * 
 * 该函数返回当前时间的ISO格式字符串，但将'T'替换为单个空格，将'Z'移除
 * 这用于在数据库或其他需要特定格式的时间表示中使用
 * 
 * @returns {string} 当前时间的ISO格式字符串，格式为YYYY-MM-DD HH:MM:SS.sss
 */
const getTimeStamp = () => new Date().toISOString().replace('T', ' ').replace('Z', '');

/**
 * 将日志信息写入到日志文件中
 * @param {string} level - 日志级别，例如 'info', 'error' 等
 * @param {string} message - 日志消息内容
 */
const logToFile = (level, message) => {
    const logMessage = `[${getTimeStamp()}] [${level.toUpperCase()}] ${message}\n`;
    fs.appendFile(getLogFilePath(), logMessage, (err) => {
        if (err) console.error('日志写入失败:', err);
    });
};

const log = {
    info: (message) => logToFile('info', message),
    warn: (message) => logToFile('warn', message),
    error: (message) => logToFile('error', message),
    debug: (message) => logToFile('debug', message),
};

module.exports = log;