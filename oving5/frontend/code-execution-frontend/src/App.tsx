import React, { useState } from "react";
import Editor from "react-simple-code-editor";
// @ts-ignore
import { highlight, languages } from "prismjs/components/prism-core";
import "prismjs/components/prism-clike";
import "prismjs/components/prism-javascript";
import Button from "react-bootstrap/Button";
import Dropdown from "react-bootstrap/Dropdown";
import DropdownButton from "react-bootstrap/DropdownButton";
import axios from "axios";
import "bootstrap/dist/css/bootstrap.min.css";

import "./App.css";

function App() {
  const [inputCode, setInputcode] = useState("");
  const [inputLang, setInputLang] = useState("");
  const [codeOutput, setCodeOutput] = useState("");

  function send() {
    axios
      .post("http://localhost:8000/code", {
        language: inputLang,
        code: inputCode,
      })
      .then(
        (response) => {
          console.log(response);
          setCodeOutput(response.data);
        },
        (error) => {
          console.log(error);
        }
      );
  }

  return (
    <>
      <Editor
        value={inputCode}
        onValueChange={(code) => setInputcode(code)}
        highlight={(code) => highlight(code, languages.js)}
        padding={10}
        style={{
          fontFamily: '"Fira code", "Fira Mono", monospace',
          fontSize: 12,
        }}
      />
      <Button variant="dark" onClick={() => send()}>
        Run
      </Button>
      <DropdownButton
        variant="dark"
        id="dropdown-basic-button"
        title="Language"
      >
        <Dropdown.Item onSelect={() => setInputLang("Python")}>
          Python
        </Dropdown.Item>
        <Dropdown.Item onSelect={() => setInputLang("Nodejs")}>
          NodeJS
        </Dropdown.Item>
        <Dropdown.Item onSelect={() => setInputLang("Go")}>GO</Dropdown.Item>
      </DropdownButton>
      <Editor
        value={codeOutput}
        onValueChange={(code) => setCodeOutput(code)}
        highlight={(code) => highlight(code, languages.js)}
        padding={10}
        style={{
          fontFamily: '"Fira code", "Fira Mono", monospace',
          fontSize: 12,
        }}
      />
    </>
  );
}

export default App;
