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
import com.kms.katalon.core.util.KeywordUtil

WebUI.openBrowser('')

WebUI.navigateToUrl('https://thegioiskinfood.com/')

WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Th Gii SkinFood - M phm chnh hng  TH G_5b2e13/a_Mu 1-17 Son Tint Nc Romand Siu L, Lu Tri _558ed2'))

WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/span_02  THM'))

WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Son Tint Nc Siu L, Lu Tri Romand Glast_ddb3ef/button_Mua Online'))

WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/a_T HNG NGAY'))

WebUI.setText(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/input_Anh_editcustomername'), 
    hoten)

WebUI.setText(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/input_Nhp h tn_editcustomerphone'), 
    sdt)

WebUI.setText(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/input_Nhp s in thoi_editcustomeremail'), 
    email)

// === Chọn Tỉnh/TP
TestObject dropdownTinh = findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/dropdownlist_tinh')

//WebUI.selectOptionByLabel(dropdownTinh, tinh, true)

//WebUI.delay(1.5)

// === Chọn Huyện
TestObject dropdownHuyen = findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/dropdownlist_huyen')

//WebUI.selectOptionByLabel(dropdownHuyen, huyen, true)

//WebUI.delay(1.5)

// === Chọn Xã
TestObject dropdownXa = findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/dropdownlist_xa')

//WebUI.selectOptionByLabel(dropdownXa, xa, true)



// === CHỌN TỈNH/THÀNH PHỐ
if (tinh != null && tinh.trim() != '' && tinh != '0') {
	WebUI.selectOptionByLabel(dropdownTinh, tinh, true, FailureHandling.OPTIONAL)
	WebUI.delay(1.5)

	// === CHỌN QUẬN/HUYỆN
	if (huyen != null && huyen.trim() != '' && huyen != '0') {
		WebUI.selectOptionByLabel(dropdownHuyen, huyen, true, FailureHandling.OPTIONAL)
		WebUI.delay(1.5)

		// === CHỌN XÃ/PHƯỜNG
		if (xa != null && xa.trim() != '' && xa != '0') {
			WebUI.selectOptionByLabel(dropdownXa, xa, true, FailureHandling.OPTIONAL)
		} else {
			println('Chưa chọn xã – bỏ qua chọn Xã.')
		}
	} else {
		println('Chưa chọn huyện – bỏ qua Huyện và Xã.')
	}
} else {
	println('Chưa chọn tỉnh – bỏ qua Tỉnh, Huyện và Xã.')
}
WebUI.setText(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/input_a ch giao hng_editcustomeraddress'),
	sonha)
//WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/div_Thanh ton tin mt khi nhn hng (COD)Min p_ae7039'))

WebUI.click(findTestObject('Object Repository/Đặt hàng/Page_Gi hng ca bn - TH GII SKINFOOD/button_THANH TON NGAY'))

//WebUI.verifyElementText(findTestObject('Object Repository/Đặt hàng/Page_TH GII SKINFOOD - n hng1002931458/h2_t hng thnh cng'), 
//  'Đặt hàng thành công')
// Đợi hệ thống phản hồi
// Đợi phản hồi từ hệ thống
WebUI.delay(2)

// Kiểm tra xem đơn hàng có thành công không
boolean isOrderSuccessful = WebUI.verifyElementPresent(findTestObject('Object Repository/Đặt hàng/Page_TH GII SKINFOOD - n hng1002931458/h2_t hng thnh cng'), 
    5, FailureHandling.OPTIONAL)

// Nếu KHÔNG thành công => kiểm tra từng trường để chỉ ra lỗi
if (!(isOrderSuccessful)) {
    println('Không thấy thông báo \'Đặt hàng thành công\'. Có thể đơn hàng đã bị lỗi.')

    if ((hoten == null) || (hoten.trim() == '')) {
        println('Họ tên bị bỏ trống.')
    }
    
    if ((sdt == null) || !(sdt.matches('^\\d{10}$'))) {
        println('Số điện thoại sai định dạng (phải đủ 10 chữ số).')
    }
    
    if ((email == null) || !(email.contains('@gmail.com'))) {
        println('Email không hợp lệ (phải chứa \'@gmail.com\').')
    }
    
    if ((sonha == null) || (sonha.trim() == '')) {
        println('Địa chỉ/số nhà bị bỏ trống.')
    }
    
    if ((tinh == null) || (tinh == '0')) {
        println('Chưa chọn Tỉnh/Thành phố.')
    }
    
    if ((huyen == null) || (huyen == '0')) {
        println('Chưa chọn Quận/Huyện.')
    }
    
    if ((xa == null) || (xa == '0')) {
        println('Chưa chọn Xã/Phường.')
    }
} else {
    println('Đặt hàng thành công.')

    WebUI.verifyElementText(findTestObject('Object Repository/Đặt hàng/Page_TH GII SKINFOOD - n hng1002931458/h2_t hng thnh cng'), 
        'Đặt hàng thành công')
}

// Đọc kết quả mong muốn từ Excel 
// === Kiểm tra thêm: nếu có hiển thị thông báo lỗi cụ thể
String actualResult = ""
TestObject successMsg = findTestObject('Object Repository/Đặt hàng/Page_TH GII SKINFOOD - n hng1002931458/h2_t hng thnh cng')
TestObject errorNotify = findTestObject('Object Repository/Đặt hàng/Page_TH GII SKINFOOD - n hng1002931458/action_notify')

// === Ưu tiên lấy thông báo thành công
if (WebUI.verifyElementPresent(successMsg, 2, FailureHandling.OPTIONAL)) {
    actualResult = WebUI.getText(successMsg).trim()
    println("Thông báo thành công: ${actualResult}")
} 
// === Nếu không thành công mà có thông báo lỗi hiển thị thì lấy nội dung
else if (WebUI.verifyElementPresent(errorNotify, 2, FailureHandling.OPTIONAL)) {
    actualResult = WebUI.getText(errorNotify).trim()
    println("Thông báo lỗi hiển thị: ${actualResult}")
} 
// === Không có thông báo gì cả, dùng logic cũ
else {
    actualResult = isOrderSuccessful ? 'Pass' : 'Fail'
    println("Không thấy thông báo hiển thị. Gán actualResult = ${actualResult}")
}

// So sánh actualResult với expected từ file Excel
if (actualResult.equalsIgnoreCase(expected.trim())) {
    println("Kết quả đúng mong đợi: ${actualResult}")
} else {
    KeywordUtil.markFailed("Kết quả KHÔNG đúng mong đợi! Thực tế: ${actualResult}, Mong đợi: ${expected}")
}


WebUI.closeBrowser()

