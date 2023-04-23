import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.amazon.com/')

WebUI.click(findTestObject('Object Repository/Page_Amazon.com. Spend less. Smile more/input_Search Amazon_field-keywords'))

WebUI.click(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_73945b'))

WebUI.click(findTestObject('Object Repository/Page_Amazon Sign-In/div_Please Enable Cookies to Continue      _3201f2'))

WebUI.setText(findTestObject('Object Repository/Page_Amazon Sign-In/input_Email or mobile phone number_email'), '7079680690')

WebUI.click(findTestObject('Object Repository/Page_Amazon Sign-In/input_Enter your email or mobile phone numb_fc7402'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Amazon Sign-In/input_Forgot your password_password'), 'rze+i5rtQXqOuUojTcS5VQ==')

WebUI.click(findTestObject('Object Repository/Page_Amazon Sign-In/input_Enter your password_signInSubmit'))

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_ea3b7c'), '3')

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_22175f'), '7')

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_73945b'), '0')

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_f57ef7'), '8')

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_eb2a81'), '0')

WebUI.setText(findTestObject('Object Repository/Page_Amazon/input_concat(For your security, we, , ve se_8b21e1'), '6')

WebUI.click(findTestObject('Object Repository/Page_Amazon/input_Please wait  44  seconds before reque_80d5ed'))

WebUI.setText(findTestObject('Object Repository/Page_Amazon.com. Spend less. Smile more/input_Search Amazon_field-keywords'), 
    'iphone 14 pro max phone case')

WebUI.closeBrowser()

