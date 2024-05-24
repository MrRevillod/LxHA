interface IconButtonProps {
    BtnClass: string,
    onClick:() => void
}

export const IconButton = ({ BtnClass, onClick }:IconButtonProps) => {
    return (
        <button className={"bg-opacity-50 hover:opacity-100 opacity-50"} onClick={onClick} >
          <i className={BtnClass}></i>  
        </button >
    )
}