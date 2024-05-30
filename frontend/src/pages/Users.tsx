
import { useState } from "react"
import { MainLayout } from "../layouts/MainLayout"
import { ModalLayout } from "../layouts/ModalLayout"
import { RegisterUserForm } from "../components/forms/RegisterUserForm"
// import { useUserStore } from "../store/UserStore"

export const UsersPage = () => {
    const [isOpen, setIsOpen] = useState<boolean>(false)

    //const { createUser } = useUserStore()
    const handleCreateUser =  () => {
        setIsOpen(true)
    }

    const handleClose = () => {
        setIsOpen(false)
    }



    return (

        <MainLayout>

            <div className="w-full  h-full flex flex-col gap-4 items-center justify-center text-neutral-950">

                <h1 className="text-5xl font-bold">Users Page</h1>

                <h2 className="text-neutral-800">This only appears if you're a administrator</h2>

                <button onClick={handleCreateUser} className="w-42 h-12 px-4 mt-4 bg-primary text-white font-bold rounded-md border-none">
                    Create User Tester
                </button>
                <ModalLayout isOpen={isOpen} onClose={handleClose}>
                        <RegisterUserForm onClose={handleClose}/>
                </ModalLayout>

            </div>

        </MainLayout>
    )
}
