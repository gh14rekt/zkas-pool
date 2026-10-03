// Synthetic encoding fixture, not a wallet address.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const {credentials}=require('../../bridge/static/js/mining-setup.js');
const zkas='zkas:p9pyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyyssmdvpfyna';
const values={host:'pool.example.com',zkas,worker:'asic_1'};
const native={mode:'native',port:5555,parentPrefix:null};
test('native credentials preserve payout and worker',()=>{
 assert.deepEqual(credentials(native,values),{url:'stratum+tcp://pool.example.com:5555',username:zkas+'.asic_1',password:'x'});
 assert.equal(credentials(native,{...values,host:'[::1]'}).url,'stratum+tcp://[::1]:5555');
});
test('malformed inputs cannot generate credentials',()=>{
 assert.throws(()=>credentials(native,{...values,zkas:zkas.slice(0,-1)+'q'}));
 for(const worker of ['other.worker','asic\nPassword: evil','<img src=x>', 'a'.repeat(65)]) assert.throws(()=>credentials(native,{...values,worker}));
 for(const host of ['https://pool.example.com','user@pool.example.com','pool.example.com:5555','x/evil']) assert.throws(()=>credentials(native,{...values,host}));
 assert.throws(()=>credentials({...native,port:0},values));
 assert.throws(()=>credentials({...native,mode:'unknown'},values));
});
