// utils/response.js
'use strict';

const response = (code, status, data, message) => {
    return {
        code,
        status: (status === 'success' || status === 'error') ? status : 'error',
        data,
        message
    }
}

const successResponse = (code, data, message) => {
    return response(code, 'success', data, message)
}

const errorResponse = (code, data, message) => {
    return response(code, 'error', data, message)
}

module.exports = { successResponse, errorResponse }