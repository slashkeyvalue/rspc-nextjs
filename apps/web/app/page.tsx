import styles from "./page.module.css";
import { FileUploader } from "./components/file-uploader/file-uploader";

export default function Page(): JSX.Element
{
    return (
        <main className={styles.main}>

            <FileUploader />
        </main>
    );
}
