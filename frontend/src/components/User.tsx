import { useParams } from "react-router-dom"
import { useUserStore } from "../store/UserStore"

interface UserProps {

}

export const User = () => {
    const { userId } = useParams()
    
    return (
        <div>
            {userId}
        </div>
    )
}