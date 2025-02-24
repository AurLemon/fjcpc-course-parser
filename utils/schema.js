/**
 * 定义返回体的 Schema
 */

const responseSchema = {
    type: 'object',
    properties: {
        code: { type: 'integer' },
        status: { type: 'string' },
        data: {
            type: 'object',
            additionalProperties: true
        },
        message: { type: 'string' }
    }
}

module.exports = { responseSchema }