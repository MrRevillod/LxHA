
import { useAuth } from "../store/AuthContext"

export const DashboardPage = () => {

    const { isAdmin } = useAuth()

    return (

        <div className="w-full h-full flex flex-col gap-4 items-center justify-center">
            <h1 className="text-5xl text-neutral-100 font-bold">Dashboard Page</h1>
            {isAdmin && <h2>This only appears if you're a administrator</h2>}
            {!isAdmin && <h2>This only appears if you're a normal user</h2>}
        </div>
    )
}