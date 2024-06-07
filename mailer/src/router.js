
import { Router } from "express"
import { authenticate } from "./middlewares.js"
import { contactFromAdmin, contactFromUser, newAccountMessageController, resetPasswordController, updateEmailController } from "./controllers.js"

export const router = Router()

router.post("/api/mailer/contact-test", authenticate("ROLE"), (req, res) => {
    res.status(200).send("Contact test successful")
})

router.post("/api/mailer/email-change", updateEmailController)
router.post("/api/mailer/reset-password", resetPasswordController)

router.post("/api/mailer/messages/from-admin", contactFromAdmin)
router.post("/api/mailer/messages/from-user", contactFromUser)

router.post("/api/mailer/new-account", newAccountMessageController)
