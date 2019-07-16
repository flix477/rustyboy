import React, {FunctionComponent, useCallback} from 'react';
import { Debugger as DebuggerType, DebugInfo } from 'rustyboy-web';

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
    <div>
      <button disabled={!debuggerRef} onClick={onContinueClick}>Continue</button>
      <button disabled={!debuggerRef} onClick={onStepInto}>Step into</button>
      <button disabled={!debuggerRef} onClick={onStepOver}>Step over</button>
    </div>
  );
};

export default Actions;