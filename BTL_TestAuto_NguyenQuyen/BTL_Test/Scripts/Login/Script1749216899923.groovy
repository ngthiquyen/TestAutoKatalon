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

WebUI.navigateToUrl('https://thegioiskinfood.com/')

WebUI.click(findTestObject('Object Repository/login/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/svg'))

WebUI.setText(findTestObject('Object Repository/login/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/input_NG NHP_customeremail'), 
    username)

WebUI.setText(findTestObject('Object Repository/login/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/input_NG NHP_customerpassword'), 
    password)

WebUI.click(findTestObject('Object Repository/login/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/button_ng nhp'))

//String actualResult = WebUI.getText(findTestObject('Object Repository/login/Page_Ti khon  TH GII SKINFOOD/error_message'))
//WebUI.verifyMatch(actualResult, expectedresult, false)
String actualResult = ''

// Kiểm tra nếu có thông báo lỗi đăng nhập
if (WebUI.waitForElementPresent(findTestObject('Object Repository/login/Page_Ti khon  TH GII SKINFOOD/error_message'), 5, 
    FailureHandling.OPTIONAL)) {
    actualResult = WebUI.getText(findTestObject('Object Repository/login/Page_Ti khon  TH GII SKINFOOD/error_message')).trim()

    WebUI.comment('Đăng nhập thất bại: ' + actualResult) // Nếu không có lỗi → kiểm tra phần chào mừng sau đăng nhập thành công
} else if (WebUI.waitForElementPresent(findTestObject('Object Repository/login/Page_Ti khon  TH GII SKINFOOD/welcome_message'), 
    5, FailureHandling.OPTIONAL)) {
    actualResult = WebUI.getText(findTestObject('Object Repository/login/Page_Ti khon  TH GII SKINFOOD/welcome_message')).trim()

    WebUI.comment('Đăng nhập thành công: ' + actualResult)
} else {
    actualResult = 'Không có thông báo thành công hoặc thất bại'

    WebUI.comment('Không xác định được trạng thái đăng nhập')
}

WebUI.verifyMatch(actualResult, expectedresult.trim(), false)

WebUI.closeBrowser()

