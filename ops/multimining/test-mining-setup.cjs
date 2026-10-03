// Address constants below are synthetic encoding fixtures, not wallet addresses.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const {credentials}=require('../../bridge/static/js/mining-setup.js');
const zkas='zkas:p9pyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyyssmdvpfyna';
const kaspa='kaspadev:qqjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzg84e95fhp';
const sedra='sedradev:qqjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgfpyysjzgyy42d4g4';
const values={host:'pool.example.com',zkas,kaspa,sedra,worker:'asic_1'};
const native={mode:'native',port:5555,parentPrefix:null};
const kas={mode:'kaspa',port:5556,parentPrefix:'kaspadev'};
const sdr={mode:'sedra',port:5557,parentPrefix:'sedradev'};
test('three modes preserve exact reward recipients and pick their own ports',()=>{
 assert.deepEqual(credentials(native,values),{url:'stratum+tcp://pool.example.com:5555',username:zkas+'.asic_1',password:'x'});
 assert.equal(credentials(kas,values).password,kaspa);
 assert.equal(credentials(sdr,values).password,sedra);
 assert.equal(credentials(sdr,{...values,host:'[::1]'}).url,'stratum+tcp://[::1]:5557');
});
test('missing, corrupted and wrong-chain parent addresses never silently fall back to pool',()=>{
 for(const invalid of ['', 'x', kaspa, sedra.slice(0,-1)+'q']) assert.throws(()=>credentials(sdr,{...values,sedra:invalid}));
 assert.throws(()=>credentials({...kas,parentPrefix:'kaspa'},values));
});
test('invalid child address, worker injection and invalid endpoint cannot generate credentials',()=>{
 assert.throws(()=>credentials(native,{...values,zkas:zkas.slice(0,-1)+'q'}));
 assert.throws(()=>credentials(native,{...values,zkas:sedra}));
 for(const worker of ['other.worker','asic\nPassword: evil','<img src=x>', 'a'.repeat(65)]) assert.throws(()=>credentials(native,{...values,worker}));
 for(const host of ['https://pool.example.com','user@pool.example.com','pool.example.com:5555','x/evil']) assert.throws(()=>credentials(native,{...values,host}));
 assert.throws(()=>credentials({...native,port:0},values));
 assert.throws(()=>credentials({...native,mode:'unknown'},values));
});
