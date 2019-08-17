import React, {FunctionComponent, useCallback} from 'react';
import { Debugger as DebuggerType, DebugInfo } from 'rustyboy-web';

import {ReactComponent as PlayIcon} from './debugger-play.svg';
import {ReactComponent as StepOverIcon} from './debugger-over.svg';
import {ReactComponent as StepIntoIcon} from './debugger-in.svg';
import './actions.css';

interface Props {
  debuggerRef: DebuggerType;
  debugInfo?: DebugInfo;
  onContinue?: () => void;
}

export const Actions: FunctionComponent<Props> = ({onContinue, debuggerRef, debugInfo}) => {
  const onContinueClick = useCallback(() => {
    if (debuggerRef && onContinue) {
      debuggerRef.continueExecution();
      onContinue();
    }
  }, [debuggerRef, onContinue]);

  const onStepInto = useCallback(() => {
    if (debuggerRef && onContinue) {
      debuggerRef.stepInto();
      onContinue();
    }
  }, [debuggerRef, onContinue]);

  const onStepOver = useCallback(() => {
    if (debuggerRef && onContinue && debugInfo) {
      debuggerRef.stepOver(debugInfo);
      onContinue();
    }
  }, [debuggerRef, onContinue, debugInfo]);

  return (
    <div className="actions">
      <button disabled={!debuggerRef} onClick={onContinueClick} title="Continue">
        <PlayIcon />
      </button>
      <button disabled={!debuggerRef} onClick={onStepOver} title="Step over">
        <StepOverIcon />
      </button>
      <button disabled={!debuggerRef} onClick={onStepInto} title="Step into">
        <StepIntoIcon />
      </button>
    </div>
  );
};

export default Actions;