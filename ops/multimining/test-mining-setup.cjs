// Synthetic encoding fixtures, not wallet addresses.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const {credentials}=require('../../bridge/static/js/mining-setup.js');
const zkas='zkas:p9pyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyyssmdvpfyna';
const kaspa='kaspadev:qqjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzg84e95fhp';
const values={host:'pool.example.com',zkas,kaspa,worker:'asic_1'};
const native={mode:'native',port:5555,parentPrefix:null};
const kas={mode:'kaspa',port:5556,parentPrefix:'kaspadev'};
test('native and Kaspa preserve independent payout settings',()=>{
 assert.deepEqual(credentials(native,values),{url:'stratum+tcp://pool.example.com:5555',username:zkas+'.asic_1',password:'x'});
 assert.equal(credentials(kas,values).password,kaspa);
});
test('missing and wrong-network parents cannot silently pay the pool',()=>{
 for(const invalid of ['', 'x', kaspa.slice(0,-1)+'q']) assert.throws(()=>credentials(kas,{...values,kaspa:invalid}));
 assert.throws(()=>credentials({...kas,parentPrefix:'kaspa'},values));
 assert.throws(()=>credentials(native,{...values,zkas:zkas.slice(0,-1)+'q'}));
 assert.throws(()=>credentials(native,{...values,worker:'other.worker'}));
 assert.throws(()=>credentials(native,{...values,host:'user@pool.example.com'}));
});
