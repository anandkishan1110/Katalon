<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Please Enable Cookies to Continue      _3201f2</name>
   <tag></tag>
   <elementGuidId>d1e6f4cb-614f-49bf-b8d9-b58e67c89c35</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#authportal-main-section</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='authportal-main-section']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>600a9d72-3c65-4b63-8f9a-68adaca0ec13</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>authportal-main-section</value>
      <webElementGuid>779fd506-c6bc-4567-9ecf-97bf40adc930</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>a-section</value>
      <webElementGuid>fdc5be7b-54e3-4fb5-b207-c348b932344b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
              






  
    
    
      Please Enable Cookies to Continue
        
          
            
          
        
      
    
  



  

















  
    






  
    
    
      
      

      




  
    
  
    
  
    
  



      
        
          
            Sign in
          
          
          
          
            
              Email or mobile phone number
            
            
            
              
                
              
              
            
            

            
            
            




  Enter your email or mobile phone number


          

          
          

          
            
            









            
            
              Continue
            

            
            
              




  By continuing, you agree to Amazon's Conditions of Use and Privacy Notice.
 

            

            

  function cf() {
    if (typeof window.uet === 'function') {
      uet('cf');
    }
    if (window.embedNotification &amp;&amp;
      typeof window.embedNotification.onCF === 'function') {
      embedNotification.onCF();
    }
  }


cf()

          

          

          

          




  
    
      Need help?
    
    
      
        



  
  
    
  


  Forgot your password?

      
    
    
      
        Other issues with Sign-In
      
    
  



          




  
  
  
    
      
      
      
        
        
      
    
  


          
          

          






        
      
    
  
  
    
    
      
        
        New to Amazon?
        
          Create your Amazon account
        
      
    
  

  
    P.when('A', 'IdentityWebAuthn', 'ready').register('webauthn', async function(A, IdentityWebAuthn) {
      /*
      This is a temporary workaround for changing the autocomplete value for the text input field. Once AUI
      adds the needed enum then we can change the value above.
      */
      A.$('#ap_email').attr(&quot;autocomplete&quot;, &quot;webauthn&quot;);
      const isCMA = await PublicKeyCredential.isConditionalMediationAvailable();
      const isPlatformSupported = await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
      if (isPlatformSupported &amp;&amp; isCMA) {
        const randomString = (Math.random() + 1).toString(36).substring(2);
        const abortController = new AbortController();

        const publicKeyCredentialRequestOptions = {
          mediation: &quot;conditional&quot;,
          signal: abortController.signal,
          publicKey: {
            challenge: Uint8Array.from(randomString, c => c.charCodeAt(0)),
            timeout: 60000,
            rpId: window.location.hostname,
            userVerification: &quot;preferred&quot;
          }
        };

        // Once implemented, we will use IdentityWebAuthn functions to invoke autofill.
        navigator.credentials.get(publicKeyCredentialRequestOptions).then(assertion => {
          const clientData = JSON.parse(new TextDecoder().decode(assertion.response.clientDataJSON));
          const authenticatorData = new TextDecoder().decode(assertion.response.clientDataJSON);

          const output = {
            clientData: clientData,
            authenticatorData: authenticatorData
          };

          console.log(output);
        }).catch(error => console.log(error));
      }
    });
  



  
  


            </value>
      <webElementGuid>c4fa263b-0f4e-4b1d-9cdd-8368b4d19495</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;authportal-main-section&quot;)</value>
      <webElementGuid>a1e1fb2f-14cd-4119-8b2e-e12b32a17ed1</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='authportal-main-section']</value>
      <webElementGuid>44b63b00-6f5b-4f2d-ad9b-9d98f098ed5c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='authportal-center-section']/div</value>
      <webElementGuid>3777901e-bad5-4d8e-a14f-d9cbb112e669</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div</value>
      <webElementGuid>fd1c0b2f-de78-4156-96ed-a32d1dc1a3cd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'authportal-main-section' and (text() = concat(&quot;
              






  
    
    
      Please Enable Cookies to Continue
        
          
            
          
        
      
    
  



  

















  
    






  
    
    
      
      

      




  
    
  
    
  
    
  



      
        
          
            Sign in
          
          
          
          
            
              Email or mobile phone number
            
            
            
              
                
              
              
            
            

            
            
            




  Enter your email or mobile phone number


          

          
          

          
            
            









            
            
              Continue
            

            
            
              




  By continuing, you agree to Amazon&quot; , &quot;'&quot; , &quot;s Conditions of Use and Privacy Notice.
 

            

            

  function cf() {
    if (typeof window.uet === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
      uet(&quot; , &quot;'&quot; , &quot;cf&quot; , &quot;'&quot; , &quot;);
    }
    if (window.embedNotification &amp;&amp;
      typeof window.embedNotification.onCF === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
      embedNotification.onCF();
    }
  }


cf()

          

          

          

          




  
    
      Need help?
    
    
      
        



  
  
    
  


  Forgot your password?

      
    
    
      
        Other issues with Sign-In
      
    
  



          




  
  
  
    
      
      
      
        
        
      
    
  


          
          

          






        
      
    
  
  
    
    
      
        
        New to Amazon?
        
          Create your Amazon account
        
      
    
  

  
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IdentityWebAuthn&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).register(&quot; , &quot;'&quot; , &quot;webauthn&quot; , &quot;'&quot; , &quot;, async function(A, IdentityWebAuthn) {
      /*
      This is a temporary workaround for changing the autocomplete value for the text input field. Once AUI
      adds the needed enum then we can change the value above.
      */
      A.$(&quot; , &quot;'&quot; , &quot;#ap_email&quot; , &quot;'&quot; , &quot;).attr(&quot;autocomplete&quot;, &quot;webauthn&quot;);
      const isCMA = await PublicKeyCredential.isConditionalMediationAvailable();
      const isPlatformSupported = await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
      if (isPlatformSupported &amp;&amp; isCMA) {
        const randomString = (Math.random() + 1).toString(36).substring(2);
        const abortController = new AbortController();

        const publicKeyCredentialRequestOptions = {
          mediation: &quot;conditional&quot;,
          signal: abortController.signal,
          publicKey: {
            challenge: Uint8Array.from(randomString, c => c.charCodeAt(0)),
            timeout: 60000,
            rpId: window.location.hostname,
            userVerification: &quot;preferred&quot;
          }
        };

        // Once implemented, we will use IdentityWebAuthn functions to invoke autofill.
        navigator.credentials.get(publicKeyCredentialRequestOptions).then(assertion => {
          const clientData = JSON.parse(new TextDecoder().decode(assertion.response.clientDataJSON));
          const authenticatorData = new TextDecoder().decode(assertion.response.clientDataJSON);

          const output = {
            clientData: clientData,
            authenticatorData: authenticatorData
          };

          console.log(output);
        }).catch(error => console.log(error));
      }
    });
  



  
  


            &quot;) or . = concat(&quot;
              






  
    
    
      Please Enable Cookies to Continue
        
          
            
          
        
      
    
  



  

















  
    






  
    
    
      
      

      




  
    
  
    
  
    
  



      
        
          
            Sign in
          
          
          
          
            
              Email or mobile phone number
            
            
            
              
                
              
              
            
            

            
            
            




  Enter your email or mobile phone number


          

          
          

          
            
            









            
            
              Continue
            

            
            
              




  By continuing, you agree to Amazon&quot; , &quot;'&quot; , &quot;s Conditions of Use and Privacy Notice.
 

            

            

  function cf() {
    if (typeof window.uet === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
      uet(&quot; , &quot;'&quot; , &quot;cf&quot; , &quot;'&quot; , &quot;);
    }
    if (window.embedNotification &amp;&amp;
      typeof window.embedNotification.onCF === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;) {
      embedNotification.onCF();
    }
  }


cf()

          

          

          

          




  
    
      Need help?
    
    
      
        



  
  
    
  


  Forgot your password?

      
    
    
      
        Other issues with Sign-In
      
    
  



          




  
  
  
    
      
      
      
        
        
      
    
  


          
          

          






        
      
    
  
  
    
    
      
        
        New to Amazon?
        
          Create your Amazon account
        
      
    
  

  
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IdentityWebAuthn&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).register(&quot; , &quot;'&quot; , &quot;webauthn&quot; , &quot;'&quot; , &quot;, async function(A, IdentityWebAuthn) {
      /*
      This is a temporary workaround for changing the autocomplete value for the text input field. Once AUI
      adds the needed enum then we can change the value above.
      */
      A.$(&quot; , &quot;'&quot; , &quot;#ap_email&quot; , &quot;'&quot; , &quot;).attr(&quot;autocomplete&quot;, &quot;webauthn&quot;);
      const isCMA = await PublicKeyCredential.isConditionalMediationAvailable();
      const isPlatformSupported = await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
      if (isPlatformSupported &amp;&amp; isCMA) {
        const randomString = (Math.random() + 1).toString(36).substring(2);
        const abortController = new AbortController();

        const publicKeyCredentialRequestOptions = {
          mediation: &quot;conditional&quot;,
          signal: abortController.signal,
          publicKey: {
            challenge: Uint8Array.from(randomString, c => c.charCodeAt(0)),
            timeout: 60000,
            rpId: window.location.hostname,
            userVerification: &quot;preferred&quot;
          }
        };

        // Once implemented, we will use IdentityWebAuthn functions to invoke autofill.
        navigator.credentials.get(publicKeyCredentialRequestOptions).then(assertion => {
          const clientData = JSON.parse(new TextDecoder().decode(assertion.response.clientDataJSON));
          const authenticatorData = new TextDecoder().decode(assertion.response.clientDataJSON);

          const output = {
            clientData: clientData,
            authenticatorData: authenticatorData
          };

          console.log(output);
        }).catch(error => console.log(error));
      }
    });
  



  
  


            &quot;))]</value>
      <webElementGuid>2f9e6175-e9c4-44e8-8381-d395a6c4d5aa</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
