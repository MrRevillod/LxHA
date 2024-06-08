
import { Show } from "../ui/Show"
import { Input } from "../ui/Input"
import { Spinner } from "../ui/Spinner"
import { TextArea } from "../ui/TextArea"
import { zodResolver } from "@hookform/resolvers/zod"
import { ModalLayout } from "../../layouts/ModalLayout"
import { MessageData } from "../../lib/types"
import { useHttpStore } from "../../store/HttpStore"
import { messageSchema } from "../../lib/schemas"
import { useModalStore } from "../../store/ModalStore"
import { useMessageStore } from "../../store/MessageStore"
import { FieldValues, SubmitHandler, useForm } from "react-hook-form"

interface MessageFormFields {
    subject: string,
    body: string
}

export const NewMessageModal = () => {

    const httpStore = useHttpStore.getState()

    const { isLoading, resetStore } = useHttpStore()
    const { fromAdminMessage, fromUserMessage } = useMessageStore()
    const { modals, setModal, data, modalVariant } = useModalStore()

    const { register, handleSubmit, formState: { errors }, reset, setError } = useForm<MessageFormFields>({
        resolver: zodResolver(messageSchema)
    })

    const handleClose = () => {
        setModal("newMessage")
        reset()
    }

    const onSubmit = async (fields: MessageFormFields) => {

        const message: MessageData = {
            subject: fields.subject,
            body: fields.body,
            from: { name: data?.name || "", email: data?.email || "" }
        }

        let status: number = 0

        if (modalVariant === "fromAdmin") {
            status = await fromAdminMessage(data?.id || "", message)

        } else if (modalVariant === "fromUser") {
            status = await fromUserMessage(data?.id || "", message)
        }

        if (status === 409 && httpStore.data.conflicts) {

            Object.entries(data.conflicts).forEach(([key, value]) => {

                setError(key as keyof MessageFormFields, {
                    type: "manual",
                    message: value?.toString()
                })
            })

            return
        }

        if (status === 200) {
            handleClose()
            resetStore()
        }

    }

    return (

        <ModalLayout isOpen={modals.newMessage} onClose={() => handleClose()}>

            <div className="w-full h-full form">

                <Show when={isLoading}>
                    <Spinner classes="fixed z-10 opacity-100" />
                </Show>

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Send email
                    </h2>
                    <p className="text-center font-normal text-neutral-900 text-sm">
                        {`Contact with ${modalVariant == "fromAdmin" ? data?.name || "" : "an Administrator"}`}
                    </p>
                </div>

                <form className="flex flex-col gap-4 px-8 w-full"
                    onSubmit={handleSubmit(onSubmit as SubmitHandler<FieldValues>)}
                >
                    <div className="flex flex-col gap-4">

                        <Input label="Subject"
                            type="text"
                            {...register("subject")}
                            placeholder="A subject with a main idea"
                            error={errors.subject ? (errors.subject.message?.toString()) : ""}
                        />

                        <TextArea label="Body"
                            {...register("body")}
                            placeholder="Write here your message"
                            error={errors.body ? (errors.body.message?.toString()) : ""}
                        />

                    </div>

                    <button type="submit" className="bg-primary text-neutral-100 rounded-lg p-2 h-11 font-bold mt-4">
                        Send
                    </button>
                </form>
            </div>
        </ModalLayout >
    )
}

