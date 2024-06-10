
import { For } from "./ui/For"
import { Link } from "react-router-dom"
import { ROLE } from "../lib/types"
import { useAuth } from "../store/AuthContext"
import { useAppStore } from "../store/AppStore"
import { useModalStore } from "../store/ModalStore"

const NavbarLinks = [
    { title: "Analitycs", to: "/analitycs", icon: "bi bi-bar-chart-fill", protected: false },
    { title: "Dashboard", to: "/dashboard", icon: "bi bi-pc-display-horizontal", protected: false },
    { title: "Users", to: "/users", icon: "bi bi-people-fill", protected: true },
]

interface NavbarLinkProps {
    to: string,
    title: string,
    icon: string,
    hidden: boolean
}

const NavbarLink = ({ to, title, icon, hidden }: NavbarLinkProps) => {

    const { role } = useAuth()
    const { modals, closeAllModals } = useModalStore()
    const { pageTitle, setPageTitle } = useAppStore()

    const classes = `${pageTitle === title ? "bg-neutral-300 bg-opacity-30" : "hover:bg-primary"}
        px-4 py-3 rounded-md text-white text-2xl ${hidden && role === ROLE.USER ? "hidden" : "flex"}
    `

    const handleClick = () => {

        if (Object.values(modals).includes(true)) {
            closeAllModals()
        }
        
        setPageTitle(title)
    }

    return (
        <Link to={to} title={title} className={classes} onClick={() => handleClick()}>
            <i className={icon}></i>
        </Link>
    )
}

export const Navbar = () => {

    const { useLogout } = useAuth()
    const { closeAllModals, modals } = useModalStore()
    const handleLogout = async () => {

        if (Object.values(modals).includes(true)) {
            closeAllModals()
        }

        await useLogout()
    }

    return (

        <nav className="h-full w-28 px-4 py-12 pt-16 flex flex-col items-center justify-between bg-primary">

            <div>
                <Link to="/"><i className="bi bi-box text-white text-4xl"></i></Link>
            </div>

            <div className="flex flex-col gap-8">
                <For of={NavbarLinks} render={(link, index) => (
                    <NavbarLink key={index} to={link.to} title={link.title} icon={link.icon} hidden={link.protected} />
                )} />
            </div>

            <div>
                <button type="button" onClick={() => handleLogout()}>
                    <i className="bi bi-box-arrow-left text-white text-2xl hover:cursor-pointer"></i>
                </button>
            </div>

        </nav>
    )
}

