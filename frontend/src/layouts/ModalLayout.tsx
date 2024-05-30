
import { Show } from "../components/ui/Show"
import { ReactNode } from "react"
import { IconButton } from "../components/ui/Button"
import { useOutsideClick } from "../lib/hooks"

interface ModalLayoutProps {
    children: ReactNode,
    isOpen: boolean,
    onClose: () => void
}

export const ModalLayout = ({ children, isOpen, onClose }: ModalLayoutProps) => {

    return (

        <Show when={isOpen}>
            <div className="fixed left-28 inset-0 flex items-center justify-center w-auto backdrop-blur-sm">
                <div className="bg-white border-0 border-opacity-10 border-neutral-600 p-6 rounded-lg">

                    <div className="flex justify-end">
                        <IconButton onClick={onClose} BtnClass="bi bi-x text-black text-4xl" />
                    </div>

                    {children}

                    {/* <div className="flex justify-end space-x-2">
                        <button onClick={onConfirm} className="px-4 py-2 bg-primary text-white rounded">Submit</button>
                        <button onClick={onClose} className="px-4 py-2 bg-gray-300 text-black rounded">Cancel</button>
                    </div> */}

                </div>
            </div>
        </Show>
    )
}