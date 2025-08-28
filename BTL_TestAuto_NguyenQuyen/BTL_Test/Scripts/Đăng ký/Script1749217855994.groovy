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

WebUI.navigateToUrl('https://thegioiskinfood.com/account/login')

WebUI.click(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/a_ng k'))

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_customerlast_name'), 
    ho)

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_customerfirst_name'), 
    ten)

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_birthday_fake'), 
    ngaysinh)

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_customeremail'), 
    email)

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_customerpassword'), 
    password)

WebUI.setText(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/input_TO TI KHON_customerphone'), 
    sdt)

WebUI.click(findTestObject('Object Repository/Đăng ký/Page_Ti khon  TH GII SKINFOOD/button_ng k'))

WebUI.setText(findTestObject(null), '')

WebUI.closeBrowser()

