
import { useUserStore } from "../store/UserStore"

export const UsersPage = () => {

    const { createUser } = useUserStore()
    const handleCreateUser = async () => await createUser()

    return (

        <div className="w-full h-full flex flex-col gap-4 items-center justify-center text-neutral-950">

            <h1 className="text-5xl font-bold">Users Page</h1>

            <h2 className="text-neutral-800">This only appears if you're a administrator</h2>

            <button onClick={() => handleCreateUser()} className="w-42 h-12 px-4 mt-4 bg-primary text-white font-bold rounded-md border-none">
                Create User Tester
            </button>

        </div>
    )
}
