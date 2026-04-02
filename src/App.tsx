import { Provider as JotaiProvider } from "jotai";
import { AppLayout } from "./layouts/app-layout";

function App() {
  return (
    <JotaiProvider>
      <AppLayout />
    </JotaiProvider>
  );
}

export default App;
