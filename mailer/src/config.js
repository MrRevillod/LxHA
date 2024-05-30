
import dotenv from "dotenv"

dotenv.config({ path: "../.env" }) // This path is relative to package.json

export const MAILER_SERVICE_URL = process.env.MAILER_SERVICE_URL || 5000
export const MAIL_ADRESS = process.env.MAILER_SERVICE_MAIL_ADRESS || "undefined"
export const MAIL_PASSWORD = process.env.MAILER_SERVICE_MAIL_PASSWORD || "undefined"
export const FROM = `LXHA ${MAIL_ADRESS}`
export const AUTH_SERVICE_URL = process.env.AUTH_SERVICE_URL || 5000

console.log(AUTH_SERVICE_URL)