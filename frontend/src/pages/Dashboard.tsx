
import { Show } from "../components/ui/Show"
import { useAuth } from "../store/AuthContext"
import { useUserStore } from "../store/UserStore"

export const DashboardPage = () => {

    const { isAdmin } = useAuth()

    return (

        <div className="w-full h-full flex flex-col gap-4 items-center justify-center">

            <h1 className="text-5xl text-neutral-100 font-bold">Dashboard Page</h1>

            <Show when={isAdmin}>
                <AdminPanel />
            </Show>

            <Show when={!isAdmin}>
                <UserPanel />
            </Show>

        </div>
    )
}

const AdminPanel = () => {

    const { createUser } = useUserStore()

    const handleCreateUser = async () => await createUser()
    // const handleUpdateUser = async () => await updateUser()

    return (

        <>
            <h2>This only appears if you're a administrator</h2>

            <div className="flex flex-row gap-4 mt-8">

                <button onClick={() => handleCreateUser()} className="w-42 h-12 bg-white px-4 mt-4 text-black font-bold rounded-md border-none">
                    Create User Tester
                </button>
                {/* <button onClick={() => handleUpdateUser()} className="w-42 h-12 bg-white px-4 mt-4 text-black font-bold rounded-md border-none">
                    Update User Tester
                </button> */}

            </div>


        </>
    )
}

const UserPanel = () => {
    return <h2>This only appears if you're a normal user</h2>
}
