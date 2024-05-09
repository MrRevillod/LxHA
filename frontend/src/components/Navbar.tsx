
import { Link } from "react-router-dom"
import { useAuthStore } from "../store/AuthStore"

export const Navbar = () => {

    const { useLogout } = useAuthStore()

    const handleLogout = async () => await useLogout()

    return (

        <>
            <nav className="flex flex-row w-full justify-between px-4">
                <div className="flex flex-row gap-4">
                    <Link to="/">Landing page</Link>
                    <Link to="/auth/login">Login page</Link>
                    <Link to="/dashboard">Client Dashboard</Link>
                    <Link to="/admin/dashboard">Admin Dashboard</Link>
                </div>
                <button onClick={() => handleLogout()}>Logout</button>
            </nav>
        </>
    )
}
