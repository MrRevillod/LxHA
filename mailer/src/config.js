

import dotenv from 'dotenv';

dotenv.config({ path: '../.env' });

export const MAILER_KEY = process.env.MAILER_KEY || "mailer-api-key"
export const MAILER_PORT = process.env.MAILER_PORT || 5000
export const MAIL_ADRESS = process.env.MAIL_ADRESS || "undefined"
export const MAIL_PASSWORD = process.env.MAIL_PASSWORD || "undefined"
export const FROM = `LXHA ${MAIL_ADRESS}`