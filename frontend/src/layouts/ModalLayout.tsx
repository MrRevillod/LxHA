
import { Show } from "../components/ui/Show"
import { ReactNode } from "react"
import { IconButton } from "../components/ui/Button"
import { useHttpStore } from "../store/HttpStore"
import { useModalStore } from "../store/ModalStore"

interface ModalLayoutProps {
    children: ReactNode,
    isOpen: boolean,
    onClose: () => void,
}

export const ModalLayout = ({ children, isOpen, onClose }: ModalLayoutProps) => {

    const { modals } = useModalStore()
    const { isLoading } = useHttpStore()

    const stopPropagation = (e: React.MouseEvent) => {
        e.stopPropagation()
    }

    return (
        <Show when={isOpen}>
            <div className="fixed left-28 inset-0 flex flex-col items-center justify-center w-auto backdrop-blur-sm right-0 top-0 bottom-0" onMouseDown={onClose}>
                <div className={`relative rounded-lg bg-white w-11/12 md:w-4/5 lg:w-3/5 2xl:w-2/5 
                    ${modals.confirmAction ? "h-1/3" : "h-4/6"} flex flex-col items-center
                    border-opacity-10 p-6 right-4 md:right-8 lg:right-10 xl:right-12
                `} onMouseDown={stopPropagation}>
                    <div className="flex justify-end w-full h-12">
                        <IconButton onClick={onClose} BtnClass="bi bi-x text-black text-4xl" />
                    </div>

                    <div className={`h-full flex flex-col items-center w-full justify-center ${isLoading ? "opacity-80" : ""}`}>
                        {children}
                    </div>

                </div>

            </div>

        </Show>
    )
}