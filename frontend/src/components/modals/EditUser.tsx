
import { ModalLayout } from "../../layouts/ModalLayout"
import { useModalStore } from "../../store/ModalStore"
import { UpdateUserForm } from "../forms/UpdateUser"

export const EditUserModal = () => {

    const { modals, setModal, data } = useModalStore()

    const handleClose = () => {
        setModal("editUser") 
    }

    return (

        <ModalLayout isOpen={modals.editUser} onClose={() => handleClose()}>

            <div className="flex flex-col gap-8 w-full">

                <div className="flex flex-col items-center gap-2">
                    <h2 className="text-4xl font-bold text-primary text-center">
                        Update account
                    </h2>

                    <p className="text-center font-normal text-neutral-900 text-sm">
                        Complete only the fields that you want to change
                    </p>
                </div>

                <UpdateUserForm user={data}/>

            </div>

        </ModalLayout>
    )
}
