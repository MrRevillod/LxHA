
import { Show } from "../ui/Show"
import { Spinner } from "../ui/Spinner"
import { ModalLayout } from "../../layouts/ModalLayout"
import { useHttpStore } from "../../store/HttpStore"
import { useModalStore } from "../../store/ModalStore"

export const ConfirmModal = () => {

    const { isLoading } = useHttpStore()
    const { modals, setModal, customAction, modalVariant } = useModalStore()

    if (!modalVariant) return null

    const handleConfirm = () => {
        if (customAction) customAction()
    }

    const subtitles: { [key: string]: string } = {
        "deleteAccount": "This account will be deleted",
        "deleteInstance": "This instance will be deleted",
        "stopInstance": "This instance will be stoped",
        "startInstance": "This instance will be started",
        "rebuildInstance": "This instance will be rebuiled",
        "rebootInstance": "This instance will be rebooted",


    }

    const subtitle = subtitles[modalVariant]

    return (

        <ModalLayout isOpen={modals.confirmAction} onClose={() => setModal("confirmAction")}>

            <div className="flex flex-col gap-10 items-center justify-center h-full">

                <Show when={isLoading}>
                    <Spinner classes="fixed z-10 opacity-100" />
                </Show>

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Confirm action
                    </h2>
                    <p className="text-center font-normal text-neutral-900 text-sm">
                        {subtitle}
                    </p>
                </div>

                <div className="flex flex-row gap-8">

                    <button onClick={() => setModal("confirmAction")} className="px-10 py-2 text-white bg-gray-500 rounded-md">Cancel</button>
                    <button onClick={() => handleConfirm()} className="px-10 py-2 text-white bg-red-500 rounded-md">Delete</button>

                </div>

            </div>

        </ModalLayout >
    )
}
