import test from 'node:test';import assert from 'node:assert/strict';import {sacCategory,contractErrorCode,normalizeSac} from './index.mjs';
test('balance code conservatively classified',()=>assert.equal(sacCategory(10),'BALANCE_CONSTRAINT'));
test('unknown code not guessed',()=>assert.equal(sacCategory(999),'UNKNOWN'));
test('parse explicit contract error only',()=>assert.equal(contractErrorCode('HostError: Error(Contract, #13)'),13));
test('host errors are not contract codes',()=>assert.equal(contractErrorCode('Error(Storage, MissingValue)'),null));
test('simulation cannot claim charged fee',()=>{let o=normalizeSac({kind:'simulation',status:'REJECTED',errorCode:11,before:{sender:'1',recipient:'0'},after:{sender:'1',recipient:'0'},feeChargedStroops:'999'});assert.equal(o.feeChargedStroops,null)});
