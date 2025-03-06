// utils/schema.js
'use strict';

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
};

module.exports = { responseSchema };
