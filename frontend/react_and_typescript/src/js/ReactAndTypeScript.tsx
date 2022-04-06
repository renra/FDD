import * as React from 'react';
import * as ReactDOM from 'react-dom';

const init = (outletId : string) : void => {
  ReactDOM.render(
    <div>
      Hello World!
    </div>,
    document.getElementById(outletId)
  );
}

declare global {
    interface Window { ReactAndTypeScript: (outletId: string) => void; }
}

window.ReactAndTypeScript = init;
