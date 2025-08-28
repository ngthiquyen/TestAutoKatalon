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

WebUI.click(findTestObject('Object Repository/Trợ giúp/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/img'))

WebUI.switchToWindowTitle('Home | Hỗ trợ khách hàng')
// Đợi trang mới load hoàn toàn
WebUI.waitForPageLoad(10)

// Xác minh URL
String currentUrl = WebUI.getUrl()

WebUI.verifyMatch(currentUrl.contains('https://hotro.thegioiskinfood.com/').toString(), 'true', true)

WebUI.click(findTestObject('Object Repository/Trợ giúp/Page_Home  H tr khch hng/span_Tr gip'))

WebUI.setText(findTestObject('Object Repository/Trợ giúp/Page_Nhm h tr  H tr khch hng/input__partner_name'), ten)

WebUI.setText(findTestObject('Object Repository/Trợ giúp/Page_Nhm h tr  H tr khch hng/input__partner_email'), email)

WebUI.setText(findTestObject('Object Repository/Trợ giúp/Page_Nhm h tr  H tr khch hng/input__name'), title)

WebUI.setText(findTestObject('Object Repository/Trợ giúp/Page_Nhm h tr  H tr khch hng/textarea_M t_description'), mota)

WebUI.click(findTestObject('Object Repository/Trợ giúp/Page_Nhm h tr  H tr khch hng/a_Gi'))

// Kiểm tra nếu 1 trong 3 trường quan trọng rỗng hoặc email sai định dạng
if (ten == '' || email == '' || title == '' || !email.contains('@gmail.com')) {
	// Expect lỗi hiển thị
	WebUI.verifyElementPresent(findTestObject('Object Repository/Trợ giúp/Page_Ticket Received  H tr khch hng/result_error'), 10)
} else {
	// Expect gửi thành công
	WebUI.verifyElementText(
		findTestObject('Object Repository/Trợ giúp/Page_Ticket Received  H tr khch hng/h1_Cm n bn  gi nh gi cho TGSF. i ng ca chng_e3380b'),
		'Cảm ơn bạn đã gửi đánh giá cho TGSF. Đội ngũ của chúng tôi sẽ xử lý ngay!'
	)
}


//WebUI.verifyElementText(findTestObject('Object Repository/Trợ giúp/Page_Ticket Received  H tr khch hng/h1_Cm n bn  gi nh gi cho TGSF. i ng ca chng_e3380b'), 
   // 'Cảm ơn bạn đã gửi đánh giá cho TGSF. Đội ngũ của chúng tôi sẽ xử lý ngay!')

WebUI.closeBrowser()

