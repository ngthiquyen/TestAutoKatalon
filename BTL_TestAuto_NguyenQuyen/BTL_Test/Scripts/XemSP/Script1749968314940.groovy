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

// Lấy tên sản phẩm ở trang chủ
String tenSanPhamTrangChu = WebUI.getText(findTestObject('Object Repository/XemSP/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/a_New Dew Son Tint Bng Merzy Dng Thch, Bn M_dade16'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/a_New Dew Son Tint Bng Merzy Dng Thch, Bn M_dade16'))

// Chờ trang chi tiết hiện ra và lấy tên sản phẩm
WebUI.waitForElementVisible(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/h1_New Dew Son Tint Bng Merzy Dng Thch, Bn _f63efa'), 
    10)

String tenSanPhamChiTiet = WebUI.getText(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/h1_New Dew Son Tint Bng Merzy Dng Thch, Bn _f63efa'))

// So sánh hai tên bằng verifyEqual
WebUI.verifyEqual(tenSanPhamTrangChu, tenSanPhamChiTiet, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_-'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_-'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_Thm vo gi'))

WebUI.verifyElementPresent(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/div_Vui lng chn bin th'), 
    0)

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/span_WD28 WHISPER ODDY'))

WebUI.click(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/button_Thm vo gi_1'))

WebUI.verifyElementText(findTestObject('Object Repository/XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/span_Sn phm  c thm vo gi hng'), 
    'Sản phẩm đã được thêm vào giỏ hàng')

TestObject cart = findTestObject('Object Repository/GioHang/cart')

WebUI.scrollToElement(cart, 5)

WebUI.waitForElementClickable(cart, 10)

WebUI.click(cart)

String tenSpGioHang= WebUI.getText(findTestObject('XemSP/Page_Son Tint Bng Merzy Dng Thch, Bn Mu Mer_5c935a/h3_giohang'))
// So sánh hai tên bằng verifyEqual
WebUI.verifyEqual(tenSanPhamChiTiet, tenSpGioHang, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.closeBrowser()

