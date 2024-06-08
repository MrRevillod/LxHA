
import { sender } from "./utils/mailer.js"
import { changeEmailTemplate, resetPasswordTemplate } from "./utils/templates.js"

export const updateEmailController = async (req, res) => {

    try {

        const { email, url } = req.body
        const subject = "LxHA - Email update request"
        
        await sender(changeEmailTemplate, subject, email, url)
        return res.status(200).json({ message: "Email sent" })

    } catch (e) {
        console.log(e)
        return res.status(500).json({ message: "Internal server error" })
    }
}

export const resetPasswordController = async (req, res) => {

    try {

        const { email, url } = req.body
        const subject = "LxHA - Password reset request"

        await sender(resetPasswordTemplate, subject, email, url)

        console.log("Mail was sent to: ", email)
        return res.status(200).json({ message: "Email sent" })

    } catch (e) {
        console.log(e)
        return res.status(500).json({ message: "Internal server error" })
    }
}

export const contactFromAdmin = async (req, res) => {

    try {

        const { subject, body, receiverEmail } = req.body
        const template = (url) => `
            <div>
                New message from Admin
            </div>
            </br>
            <div>
                ${body}
            </div>
        `

        await sender(template, subject, receiverEmail, null)

        console.log("Mail sent to: ", receiverEmail)
        return res.status(200).json({ message: "Email sent" })

    } catch (e) {
        console.log(e)
        return res.status(500).json({ message: "Internal server error" })
    }
}

export const contactFromUser = async (req, res) => {

    try {

        const { receiverEmail, fromEmail, fromName, subject, body } = req.body
        const template = (url) => `
            <div>
                New message from user ${fromName} - ${fromEmail}
            </div>
            </br>
            <div>
                ${body}
            </div>
        `

        await sender(template, subject, receiverEmail, null)

        console.log("Mail was sent to: ", receiverEmail)
        return res.status(200).json({ message: "Email sent" })

    } catch (e) {
        console.log(e)
        return res.status(500).json({ message: "Internal server error" })
    }
}

export const newInstanceMessageController = async (req, res) => {
    return res.status(200).json({ message: "OK" })
}

export const newAccountMessageController = async (req, res) => {

    try {

        const { email, password } = req.body
        const subject = "LxHA - New Account"
        const template = (url) => `<div>${email}</div><br/><div>${password}</div>`

        await sender(template, subject, email, null)

        console.log("Mail was sent to: ", email)
        return res.status(200).json({ message: "Email sent" })

    } catch (e) {
        console.log("error new account", e)
        return res.status(500).json({ message: "Internal server error" })
    }
}
