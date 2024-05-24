import { ReactNode } from "react"
import { Show } from "../components/ui/Show"
import { useOutsideClick } from "../lib/hooks"
import { IconButton } from "../components/ui/Button"

interface ModalLayoutProps {
    children: ReactNode, 
    isOpen: boolean,
    onConfirm: () => void,
    onClose: () => void
}

export const  ModalLayout= ({ children, isOpen, onConfirm, onClose }: ModalLayoutProps) => {

    const ref = useOutsideClick(onClose)

    return (
        <Show when={isOpen} >
            <div   className="fixed left-28 inset-0 flex items-center justify-center    backdrop-blur-sm ">
                <div ref={ref} className="bg-white  border-1 border-opacity-10 border-neutral-600 p-6 rounded-lg">

                    <div className="flex justify-end  ">
                        <IconButton  onClick={onClose} BtnClass="bi bi-x text-black text-4xl"/>
                    </div>

                    {children}
                    
                    <div className="flex justify-end space-x-2">
                        <button onClick={onConfirm} className="px-4 py-2 bg-primary text-white rounded">Submit</button>
                        <button onClick={onClose} className="px-4 py-2 bg-gray-300 text-black rounded">Cancel</button>
                    </div>

                </div>
            </div>
        </Show>
    )
}