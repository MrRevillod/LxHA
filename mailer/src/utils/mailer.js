
import { createTransport } from "nodemailer"
import { MAIL_ADRESS, MAIL_PASSWORD } from "../config.js"

export const transporter = createTransport({
    service: "gmail",
    auth: {
        user: MAIL_ADRESS,
        pass: MAIL_PASSWORD
    }
})

export const mailValidator = (mail) => {
    const regex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/
    return regex.test(mail)
}

export const sender = async (template, subject, email, url) => {

    try {
        
        if (!mailValidator(email)) {
            return { code: 400, message: "Invalid email" }
        }

        const options = {
            from: `LXHA ${MAIL_ADRESS}`,
            to: email,
            subject,
            html: template(url)
        }

        return await transporter.sendMail(options)

    } catch (error) {
        console.error("Error:", error)
        throw new Error(error)
    }
}
